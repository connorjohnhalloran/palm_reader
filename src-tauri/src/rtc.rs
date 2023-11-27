use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::ice_transport::ice_candidate::{RTCIceCandidate, RTCIceCandidateInit};
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::sdp::sdp_type::RTCSdpType;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

use crate::MyState;

#[tauri::command]
pub async fn create_offer(state: State<'_, MyState>, app_handle: AppHandle) -> Result<(), ()> {
    let mut m = MediaEngine::default();
    m.register_default_codecs().expect("Codec issue");

    let mut registry = Registry::new();
    registry = register_default_interceptors(registry, &mut m).expect("123");

    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    let peer_connection = Arc::new(api.new_peer_connection(config).await.expect("fsjdlj"));
    let h1 = app_handle.app_handle();

    peer_connection.on_ice_candidate(Box::new(move |candidate: Option<RTCIceCandidate>| {
        let h2 = h1.app_handle();
        match candidate {
            Some(c) => Box::pin(async move {
                h2.emit_all("register_client_ice_candidate", c.to_json().unwrap())
                    .expect("Error when emitting.");
            }),
            None => Box::pin(async {}),
        }
    }));

    // Create data channel on peer connection
    let data_channel = peer_connection
        .create_data_channel("data", None)
        .await
        .expect("Yikes");

    // Add what we need to the Tauri managed state
    *(state.0.lock().unwrap()) = Some(peer_connection.clone());
    *(state.1.lock().unwrap()) = Some(data_channel.clone());

    // Create and send offer for frontend
    let offer = peer_connection.create_offer(None).await.unwrap();
    peer_connection.set_local_description(offer).await.unwrap();
    let local_description = peer_connection.local_description().await.unwrap();
    app_handle
        .emit_all("backend_offer_ready", local_description.sdp)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn remote_ready(answer: String, state: State<'_, MyState>) -> Result<(), ()> {
    let mut sd = RTCSessionDescription::default();
    sd.sdp = answer;
    sd.sdp_type = RTCSdpType::Answer;

    let pc = (*(state.0.lock().unwrap())).as_ref().unwrap().clone();
    pc.set_remote_description(sd).await.unwrap();

    //let z = x.get_senders().await[0].track().await.unwrap();
    //let s = x.get_senders().await[0].clone();
    //s.send(parameters)
    //let t = s.track().await.unwrap().clone();
    //println!("{}", x.remote_description().await.unwrap().sdp);

    //pc.gathering_complete_promise().await.recv().await;
    //pc.add_ice_candidate(candidate);

    /*
    thread::sleep(Duration::from_millis(500));
    println!("Sending Message");
    let message = &Bytes::from_static(b"Hello!");
    let r = dc.send(message).await.expect("Something happened");
    */
    //pc.remote_description();

    Ok(())
}

#[tauri::command]
pub async fn register_ice(
    candidate: String,
    sdp_mid: Option<String>,
    sdp_mline_index: Option<u16>,
    username_fragment: Option<String>,
    state: State<'_, MyState>,
) -> Result<(), ()> {
    let pc = (*(state.0.lock().unwrap())).as_ref().unwrap().clone();

    let c = RTCIceCandidateInit {
        candidate,
        sdp_mid,
        sdp_mline_index,
        username_fragment,
    };

    pc.add_ice_candidate(c)
        .await
        .expect("Error adding ice candidate");

    Ok(())
}
