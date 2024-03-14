use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <Window>{

            TwoButtons = {{TwoButtons}} {
                button_0: {},
                button_1: {
                    color: #0f0,
                }
            }

            my_button = <Button> {
                text: "Hello, World"
            }
        }
    }


}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

#[derive(Live, LiveHook)]
pub struct TwoButtons {
    #[live] button_0: Button,
    #[live] button_1: Button,
}

impl LiveRegister for TwoButtons {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, _cx: &mut Cx, actions:&Actions) {
        if self.ui.button(id!(button_1)).clicked(&actions) {
            println!("CLICKED!");
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}