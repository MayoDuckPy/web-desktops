use leptos::html::{Div, Video};
use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use leptos_meta::*;
use std::time::Duration;

#[component]
pub fn VideoPlayer() -> impl IntoView {
    // TODO: Extract directories from leptos_options for str formatting
    // Video player
    // let video_player_modal = create_node_ref::<Dialog>();
    let video_player_element = create_node_ref::<Div>();
    let video_element = create_node_ref::<Video>();

    // Video player button/imgs
    let playback_img = RwSignal::new("/icons/play-button.svg");
    let volume_img = RwSignal::new("/icons/volume-button.svg");
    let volume = RwSignal::new(1.0);

    // Handle video ui disappearing after inactivity
    let lock_ui = RwSignal::new(false);
    let lock_cursor = RwSignal::new(false);
    let enable_video_ui = RwSignal::new(true);
    let enable_video_cursor = RwSignal::new(true);
    let inactive_duration = Duration::from_millis(1250); // 1.25s
    let mut video_ui_timer: Option<TimeoutHandle> = None;
    let timeout_callback = move || {
        // Cursor locked; UI should stay unchanged
        if lock_cursor.get_untracked() {
            return;
        }

        enable_video_cursor.set(false);
        if enable_video_ui.get_untracked() {
            enable_video_ui.set(false);
        }
    };

    view! {
        <Script type_="module">
        r#"
            import { connectClient, createServer, closeConnection } from '/scripts/stream.js';

            window.onload = () => {
                // Stream connectivity
                document.getElementById('closeConnection').onclick = closeConnection;
                document.getElementById('serverConnect').onclick = createServer;
                document.getElementById('clientConnect').onclick = async () => {
                    await connectClient();
                    document.getElementById('videoPlayerModal').showModal();
                };

                // Video controls
                document.querySelector('#videoControls .exitButton').onclick = () => {
                    closeConnection();
                    document.getElementById('videoPlayerModal').close();
                };
            };
        "#
        </Script>

        <dialog id="videoPlayerModal">
            <div id="videoPlayer" node_ref=video_player_element>
                <div
                    id="videoControls"
                    data-ui_visible=enable_video_ui
                >
                    <div
                         class="topControls"
                         on:mouseenter=move |_| { lock_cursor.set(true); }
                         on:mouseleave=move |_| { lock_cursor.set(false); }
                    >
                    // [Exit], [], []
                        <button class="exitButton">
                            <img
                                alt="Terminate video stream"
                                src="/icons/exit-button.svg"
                                height=20
                                width=20
                            >
                            </img>
                        </button>
                    </div>
                    <div class="bottomControls"
                         on:mouseenter=move |_| { lock_cursor.set(true); }
                         on:mouseleave=move |_| { lock_cursor.set(false); }
                    >
                    // [Play/Pause, Volume, VolumeSlider], [], [Fullscreen]
                        <button
                             class="playbackState"
                             on:click=move |_| { toggle_playback(video_element); }
                        >
                            <img
                                alt="Toggle playback"
                                prop:src=playback_img
                                height=20
                                width=20
                            >
                            </img>
                        </button>
                        <button
                             class="volumeButton"
                             on:click=move |_| { toggle_mute(volume); }
                        >
                            <img
                                alt="Volume"
                                prop:src=volume_img
                                height=24
                                width=24
                            >
                            </img>
                        </button>
                        <input
                             class="volumeSlider"
                             type="range"
                             min=0
                             max=1
                             prop:value=volume
                             step=0.01
                             on:input=move |ev| {
                                 if let Ok(vol) = event_target_value(&ev).as_str().parse() {
                                     volume.set(vol);
                                 }
                             }
                        >
                        </input>
                        <button
                            class="fullscreenButton"
                            on:click=move |_| { toggle_fullscreen(video_player_element); }
                        >
                            <img
                                 alt="Toggle fullscreen"
                                 src="/icons/fullscreen.svg"
                                 height=20
                                 width=20
                            >
                            </img>
                        </button>
                    </div>
                </div>
                <video
                    autoplay
                    prop:volume=volume
                    node_ref=video_element
                    style:cursor=move || { if enable_video_cursor.get() { "auto" } else { "none" }}
                    on:play=move |_| { playback_img.set("/icons/pause-button.svg"); }
                    on:pause=move |_| { playback_img.set("/icons/play-button.svg"); }
                    on:volumechange=move |_| {
                        let img = match volume.get_untracked() > 0.0 {
                            true  => "/icons/volume-button.svg",
                            false => "/icons/volume-button-mute.svg",
                        };
                        volume_img.set(img);
                    }
                    on:click=move |_| { toggle_playback(video_element); }
                    on:dblclick=move |_| { toggle_fullscreen(video_player_element); }
                    on:mouseleave=move |_| { if let Some(timer) = video_ui_timer { timer.clear(); }}
                    on:mousemove=move |_| {
                        // Cursor locked; do not create more timers
                        // NOTE: Could clear timer above but better to finish
                        //       current one as event will fire even while over controls
                        if lock_cursor.get_untracked() { return; }

                        // Set only if necessary to prevent useless signalling
                        if !enable_video_cursor.get_untracked() {
                            enable_video_cursor.set(true);
                        }

                        if !lock_ui.get_untracked() {
                            enable_video_ui.set(true);
                        }

                        // Clear old timer, if any
                        if let Some(timer) = video_ui_timer {
                            timer.clear();
                        }

                        // Try to set new timer otherwise ignore the error (*probably* fine)
                        // NOTE: We can use time since last event to prevent useless timers
                        if let Ok(timer_handle) = set_timeout_with_handle(timeout_callback, inactive_duration) {
                            video_ui_timer = Some(timer_handle);
                        }
                    }
                >
                </video>
            </div>
        </dialog>
    }
}

fn toggle_fullscreen(video_player_element: NodeRef<Div>) {
    let video_player = match video_player_element() {
        None => return,
        Some(video_player) => video_player,
    };

    if document().fullscreen_element() == None {
        let _ = video_player.request_fullscreen();
        return;
    }

    document().exit_fullscreen();
}

fn toggle_playback(video_element: NodeRef<Video>) {
    let video = match video_element() {
        None => return,
        Some(video) => video,
    };

    if video.paused() || video.ended() {
        let _ = video.play();
    } else {
        let _ = video.pause();
    }
}

fn toggle_mute(volume: RwSignal<f64>) {
    let vol = match volume.get_untracked() > 0.0 {
        true => 0.0,
        false => 1.0,
    };
    volume.set(vol);
}
