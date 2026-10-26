// SPDX-License-Identifier: GPL-3.0-or-later
#define _GNU_SOURCE
#include <pthread.h>
#include <stdint.h>
#include <stdlib.h>

#include <spa/param/audio/format-utils.h>
#include <pipewire/pipewire.h>

typedef void (*radiant_process_fn)(void *userdata, float *output, uint32_t frames,
                                   uint32_t channels);
typedef void (*radiant_error_fn)(void *userdata);

struct radiant_pw_stream {
    struct pw_main_loop *loop;
    struct pw_stream *stream;
    struct spa_hook listener;
    pthread_t thread;
    radiant_process_fn process;
    radiant_error_fn error;
    void *userdata;
    uint32_t channels;
    int thread_started;
};

static void on_process(void *userdata) {
    struct radiant_pw_stream *handle = userdata;
    struct pw_buffer *pw_buffer = pw_stream_dequeue_buffer(handle->stream);
    if (pw_buffer == NULL)
        return;
    struct spa_buffer *buffer = pw_buffer->buffer;
    if (buffer->n_datas == 0 || buffer->datas[0].data == NULL) {
        pw_stream_queue_buffer(handle->stream, pw_buffer);
        return;
    }
    const uint32_t stride = sizeof(float) * handle->channels;
    uint32_t frames = buffer->datas[0].maxsize / stride;
    if (pw_buffer->requested > 0 && pw_buffer->requested < frames)
        frames = (uint32_t)pw_buffer->requested;
    handle->process(handle->userdata, buffer->datas[0].data, frames,
                    handle->channels);
    buffer->datas[0].chunk->offset = 0;
    buffer->datas[0].chunk->stride = (int32_t)stride;
    buffer->datas[0].chunk->size = frames * stride;
    pw_stream_queue_buffer(handle->stream, pw_buffer);
}

static void on_state_changed(void *userdata, enum pw_stream_state old,
                             enum pw_stream_state state, const char *message) {
    (void)old;
    (void)message;
    struct radiant_pw_stream *handle = userdata;
    if (state == PW_STREAM_STATE_ERROR && handle->error != NULL)
        handle->error(handle->userdata);
}

static const struct pw_stream_events stream_events = {
    PW_VERSION_STREAM_EVENTS,
    .state_changed = on_state_changed,
    .process = on_process,
};

static void *run_loop(void *userdata) {
    struct radiant_pw_stream *handle = userdata;
    pw_main_loop_run(handle->loop);
    return NULL;
}

struct radiant_pw_stream *radiant_pw_start(uint32_t rate, uint32_t channels,
                                           radiant_process_fn process,
                                           radiant_error_fn error,
                                           void *userdata) {
    if (rate == 0 || channels == 0 || process == NULL)
        return NULL;
    pw_init(NULL, NULL);
    struct radiant_pw_stream *handle = calloc(1, sizeof(*handle));
    if (handle == NULL)
        return NULL;
    handle->channels = channels;
    handle->process = process;
    handle->error = error;
    handle->userdata = userdata;
    handle->loop = pw_main_loop_new(NULL);
    if (handle->loop == NULL)
        goto fail;

    struct pw_properties *properties = pw_properties_new(
        PW_KEY_MEDIA_TYPE, "Audio", PW_KEY_MEDIA_CATEGORY, "Playback",
        PW_KEY_MEDIA_ROLE, "Production", PW_KEY_APP_NAME, "Radiant", NULL);
    handle->stream = pw_stream_new_simple(
        pw_main_loop_get_loop(handle->loop), "Radiant output", properties,
        &stream_events, handle);
    if (handle->stream == NULL)
        goto fail;

    uint8_t pod_buffer[1024];
    struct spa_pod_builder builder = SPA_POD_BUILDER_INIT(pod_buffer, sizeof(pod_buffer));
    struct spa_audio_info_raw info = SPA_AUDIO_INFO_RAW_INIT(
        .format = SPA_AUDIO_FORMAT_F32,
        .channels = channels,
        .rate = rate);
    const struct spa_pod *params[1];
    params[0] = spa_format_audio_raw_build(&builder, SPA_PARAM_EnumFormat, &info);
    int result = pw_stream_connect(
        handle->stream, PW_DIRECTION_OUTPUT, PW_ID_ANY,
        PW_STREAM_FLAG_AUTOCONNECT | PW_STREAM_FLAG_MAP_BUFFERS |
            PW_STREAM_FLAG_RT_PROCESS,
        params, 1);
    if (result < 0)
        goto fail;
    if (pthread_create(&handle->thread, NULL, run_loop, handle) != 0)
        goto fail;
    handle->thread_started = 1;
    return handle;

fail:
    if (handle->stream != NULL)
        pw_stream_destroy(handle->stream);
    if (handle->loop != NULL)
        pw_main_loop_destroy(handle->loop);
    free(handle);
    pw_deinit();
    return NULL;
}

void radiant_pw_stop(struct radiant_pw_stream *handle) {
    if (handle == NULL)
        return;
    if (handle->thread_started) {
        pw_main_loop_quit(handle->loop);
        pthread_join(handle->thread, NULL);
    }
    if (handle->stream != NULL) {
        pw_stream_disconnect(handle->stream);
        pw_stream_destroy(handle->stream);
    }
    if (handle->loop != NULL)
        pw_main_loop_destroy(handle->loop);
    free(handle);
    pw_deinit();
}
