# Makepad Framework

The architecture of Makepad Framework is built upon a 3 layer system: Makepad Widgets, Makepad Draw,and Makepad Platform.

Makepad Widgets is the main widget crate which provides standard UI components for development. It sits on top of Makepad Draw.
Makepad Draw is the drawing layer and sits above Makepad Platform.
Makepad Platform is the main platform abstraction layer which interfaces with the various hardware device platforms.

## Makepad Widgets

The Makepad Widgets provides a set of commonly used UI elements toolkit that works with the Makepad ecosystem as part of the Makepad Framework. It is integrated with the Makepad Live Visual Designer and Code Editor to allow manual or visual editing.

It allows application developers to easily use these UI widget components to build and compose their UI application.

It contains:

- a retained mode widget system designable from the DSL
- a set of base widgets (buttons, sliders, checkboxes, lists etc.)

### Widgets List

The folowing is a list of the supported widgets:

* Window
* View
    * Horizontal Layout
    * Vertical Layout
* Text Label
* Button
* Image (JPG, PNG)
* Text Input
* Check Box
* Color Picker
* Slider
* Flat List
* Dock
* Dropdown
* File Tree
* ...

More on [Makepad Widgets](widgets.html)

## Makepad Draw

Makepad's 2D drawing layer.

It contains:

- the immediate mode 2D context
- the turtle layout system
- the font engine
- the vector engine
- image decoding
- base shaders
- the shader standard library

More on [Makepad Draw](draw.html)

## Makepad Platform

Makepad’s main platform abstraction layer.

It contains:

- the windowing system with keyboard, mouse and touch input
- the live system
- the shader compiler / graphics APIs
- networking
- the video capture APIs
- the audio APIs

More on [Makepad Platform](platform.html)