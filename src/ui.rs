/// Contain the UI of the timer
use iced::{
    button, executor, time, Align, Application, Button, Clipboard, Column, Command, Element,
    Settings, Subscription, Text,
};
use notify_rust::{Notification, Timeout};
// use mac_notification_sys::*;
use tracing::info;

use crate::{cmd::TimerOpt, parse::TimeoutDuration};

#[derive(Default)]
struct Timer {
    value: TimeoutDuration,
    title: String,
    stop: button::State,
    pause: bool,
    notified: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Stop,
    Tick,
    Start,
}

impl Application for Timer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = TimerOpt;

    fn new(flag: TimerOpt) -> (Self, Command<Message>) {
        let mut timer = Timer::default();
        timer.value = flag.duration;
        timer.title = flag.title;
        (timer, Command::none())
    }

    fn title(&self) -> String {
        format!("Timer")
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {
                if !self.pause {
                    self.value.tick();
                }
                if self.value.is_timeout() && !self.notified {
                    self.notified = true;
                    self.notify();
                }
            }
            Message::Stop => {
                info!("Stop message received");
                // TODO: send message to timer to stop it
                self.pause = true;
            }
            Message::Start => {
                info!("Start message received");
                self.pause = false;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        // TODO: figure out how to center the text & button
        let button_label = self.button_label();
        let msg = self.on_press_command();
        Column::new()
            .padding(20)
            .push(Text::new(&self.title).size(30))
            .push(Text::new(&self.value).size(50))
            .push(Button::new(&mut self.stop, Text::new(button_label)).on_press(msg))
            .align_items(Align::Center)
            .into()
    }
}

impl Timer {
    fn on_press_command(&self) -> Message {
        if self.pause {
            Message::Start
        } else {
            Message::Stop
        }
    }
    fn button_label(&self) -> String {
        if self.pause {
            "Start".to_owned()
        } else {
            "Pause".to_owned()
        }
    }

    fn notify(&self) {
        // TODO: extend sound library to be able to play different sound
        Notification::new()
            .appname("Timer")
            .summary("Time is over")
            .body(&self.value.summary())
            .sound_name("Glass")
            // Mac doesn't work as expected
            .timeout(Timeout::Never) // this however is
            .show()
            .unwrap();
    }
}

pub fn start(opt: TimerOpt) -> iced::Result {
    let mut setting = Settings::default();
    setting.window.size = (250, 150);
    setting.flags = opt;
    Timer::run(setting)
}
