# Makepad Widgets

Makepad’s main widget crate stacking on top of Makepad Draw.

It contains:

- a retained mode widget system designable from the DSL
- a set of base widgets (buttons, sliders, checkboxes, lists etc.)

## Walk-through

The foundation on which all Makepad widgets are built:

- A `DesktopWindow` widget to create a window
- The ability to hook the drawflow to immediate mode for drawing

Widgets draw themselves when `widget_draw` is called on them. To accomplish this, they use a mechanism identical to the one used for drawing the red rectangle shown earlier. Therefore, even though widgets draw themselves using an "immediate mode," they can still form retained mode structures that are designable from the DSL.

**Example: A window with a single button**

```rust
use makepad_widgets::*;

live_design!{
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::button::Button;
    App = {{App}} {
        ui:<DesktopWindow>{
	            my_button = <Button>{label:”hello”}
        }
    }
}

impl AppMain for App{
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
				}
				let actions = self.ui.handle_widget_event(cx, event);
				if self.ui.get_button(id!(my_button)).clicked(&actions) {
				    println!(“CLICKED!”);
				}
    }
}
```

*Retained mode* structures reside within the `<View>{}` widget, which is a component that can have a DSL-defined set of child widgets. All child widgets implement the `dyn` trait, and can be named and queried on the widget-tree.

Makepad's UI DSL does not rely on a single, giant interpreter. Instead, it is defined per widget. Therefore, every widget can determine its own rendering model, whether it be immediate mode, retained mode, or often a mixture of both.

In this example, `DesktopWindow` inherits from `View`, which gives it the ability to add arbitrary child widgets. The root widget (`DesktopWindow`) handles the incoming event stream followed by a button-query to check if it was clicked.

This makes a *polling architecture* of the event flow possible. This allows for closure and borrow-free event handling as opposed to what using a callback closure would impose.

In this example the root widget `DesktopWindow` handles the incoming event stream, and then the button is queried to check if it was clicked. This model enables a *polling architecture* for the event flow, which makes it possible to handle events without using closure or borrowing, instead of imposing a callback closure.

## Queries

The `self.ui.get_button(id!(my_button))` call queries the UI structure which then returns a `ButtonRef` type.
For each button, there is a `ButtonRef` and `ButtonSet` type that wraps a nullable reference to one or more buttons. This allows queries on multiple buttons at once, so only a single event handler is required. For instance `self.ui.get_button_set(ids!(my_button))` would return all buttons called `my_button` . 

This is particularly useful for multi-device UIs.

## Using the turtle for layout

All widgets are laid out using the same turtle, as they are drawn using the same immediate mode drawflow.

For instance a button can be laid out like this:

```rust
my_button = <Button>{
	walk:{width:100,height:100},
	label:”hello”
}
```

![turtle_100px.png](images/turtle_100px.png)

The `walk` structure properties define a fixed-size `button`, with a height and width of 100px each.

---

```rust
my_button = <Button>{
	walk:{width:Fit},
	label:”hello”
}
```

![turtle_fit.png](images/turtle_fit.png)

Setting the `button` dimensions to `Fit` like this, makes the container snap to the `label` text’s dimensions.

---

Similarly many UI components with child nodes follow this model.

```rust
<View>
	{layout:{flow:Right, spacing:50}
	<Button>{label:”B1”},
	<Button>{label:”B2”}
}
```

![turtle_frame.png](images/turtle_frame.png)

The implementation method of widgets doesn’t matter since the layout model is inside the immediate mode structure. It works regardless of whether it uses a retained mode or an entirely immediate mode with a hookpoint.

## States and animation

Many widgets use a `LiveState` object to handle state animations.

**Example** Creating a `hover` state for a single `draw_bg` shader

**Example** Creating a `hover` state for a single `draw_bg` shader.

```rust
Live_design!{
  
    Button= {{Button}} {
       draw_bg: {
            instance hover: 0.0
            fn pixel(self) -> vec4 { return mix(#f00, #0f0, self.hover) }
        }
        state: {
            hover = {
							  default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 0.0}
                    }
                }
                
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 1.0}
                    }
                }
            }
         }
     } 
}

#[derive(Live)]
pub struct Button {
    #[state] state: LiveState,
    …
}

impl Button {
    pub fn handle_event_with(&mut self, cx: &mut Cx, event: &Event, dispatch_action: &mut dyn FnMut(&mut Cx, ButtonAction)) {
        self.state_handle_event(cx, event);
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerHoverIn(_) => {
                self.animate_state(cx, id!(hover.on));
            },
            …
```

This example contains the code needed to animate to the `hover.on` state when a mouse hover occurs.

The state and animation system uses the information provided by the shader-compiler to know where to write the hover value in existing draw structures.

So this way of handling user interaction

- does not require a regenerate or redraw of the UI data
- can run entirely inside a `MouseMove` event
- does not require the system to emit a `Draw` event

However, the system can also be instructed to call `redraw` to animate properties that require a real redraw i. e. when animating or changing the `walk` or `layout` properties.
