use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use radiant_plugins::{
    HostRequest, HostResponse, PluginDescriptor, PluginFormat, PluginRole, decode_protocol,
    encode_protocol,
};

#[test]
fn worker_processes_reference_commands_out_of_process() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_radiant-plugin-worker"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("worker");
    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let mut reader = BufReader::new(stdout);
    let descriptor = PluginDescriptor {
        format: PluginFormat::Lv2,
        role: PluginRole::Effect,
        plugin_id: "org.test".into(),
        name: "Test".into(),
        path: "reference.lv2".into(),
        audio_inputs: 2,
        audio_outputs: 2,
        parameters: 1,
        supports_state: true,
        supports_ui: false,
        latency_frames: 0,
        tail_frames: 0,
    };
    let requests = [
        HostRequest::Instantiate(descriptor),
        HostRequest::SetParameter {
            id: "gain".into(),
            value: 0.5,
        },
        HostRequest::Process { frames: 4 },
        HostRequest::Shutdown,
    ];
    for request in requests {
        stdin
            .write_all(&encode_protocol(&request).expect("encode"))
            .expect("write");
        stdin.flush().expect("flush");
        let mut line = String::new();
        reader.read_line(&mut line).expect("response");
        let response: HostResponse = decode_protocol(line.trim_end().as_bytes()).expect("decode");
        assert!(matches!(
            response,
            HostResponse::Ready | HostResponse::Processed { frames: 4 }
        ));
    }
    assert!(child.wait().expect("wait").success());
}
