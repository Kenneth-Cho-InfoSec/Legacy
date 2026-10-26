use std::io::{self, BufRead, Write};

use radiant_plugins::{
    ClapHost, HostRequest, HostResponse, Lv2Host, NativePluginInstance, PluginFormat,
    ProcessContext, Vst3Host, decode_protocol, encode_protocol,
};

fn main() {
    let stdin = io::stdin();
    let mut instance: Option<NativePluginInstance> = None;
    for line in stdin.lock().lines().map_while(Result::ok) {
        let request: HostRequest = match decode_protocol(line.as_bytes()) {
            Ok(request) => request,
            Err(error) => {
                write_response(&HostResponse::Error(error.to_string()));
                continue;
            }
        };
        let response = match request {
            HostRequest::Instantiate(descriptor) => {
                let result = match descriptor.format {
                    PluginFormat::Lv2 => Lv2Host::instantiate(descriptor),
                    PluginFormat::Clap => ClapHost::instantiate(descriptor),
                    PluginFormat::Vst3 => Vst3Host::instantiate(descriptor),
                };
                result.map_or_else(
                    |error| HostResponse::Error(error.to_string()),
                    |value| {
                        instance = Some(value);
                        HostResponse::Ready
                    },
                )
            }
            HostRequest::SetParameter { id, value } => instance
                .as_mut()
                .ok_or_else(|| "worker is not instantiated".to_string())
                .and_then(|instance| {
                    instance
                        .set_parameter(&id, value)
                        .map_err(|error| error.to_string())
                })
                .map_or_else(HostResponse::Error, |()| HostResponse::Ready),
            HostRequest::SaveState => instance.as_ref().map_or_else(
                || HostResponse::Error("worker is not instantiated".into()),
                |instance| HostResponse::State(instance.save_state()),
            ),
            HostRequest::RestoreState(state) => instance
                .as_mut()
                .ok_or_else(|| "worker is not instantiated".to_string())
                .and_then(|instance| {
                    instance
                        .restore_state(state)
                        .map_err(|error| error.to_string())
                })
                .map_or_else(HostResponse::Error, |()| HostResponse::Ready),
            HostRequest::Process { frames } => instance
                .as_mut()
                .ok_or_else(|| "worker is not instantiated".to_string())
                .and_then(|instance| {
                    let input = vec![0.0; frames as usize];
                    let mut output = vec![0.0; frames as usize];
                    instance
                        .process(
                            &[&input],
                            &mut [&mut output],
                            &ProcessContext {
                                sample_rate: 48_000,
                                frames: frames as usize,
                                midi: &[],
                            },
                        )
                        .map_err(|error| error.to_string())
                })
                .map_or_else(
                    |error| HostResponse::Error(error.clone()),
                    |()| HostResponse::Processed { frames },
                ),
            HostRequest::Shutdown => {
                write_response(&HostResponse::Ready);
                break;
            }
        };
        write_response(&response);
    }
}

fn write_response(response: &HostResponse) {
    let mut stdout = io::stdout().lock();
    if let Ok(bytes) = encode_protocol(response) {
        let _ = stdout.write_all(&bytes);
        let _ = stdout.flush();
    }
}
