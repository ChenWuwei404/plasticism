# Plasticism

Plasticism /plæˈstɪsɪzəm/ is a widget library that can be easily customized.

> [!WARNING]
> 
> `plasticism` is still under development.

## Quick Start

todo!()

## Library Struct and Usages

### `plasticism::core`

Basic payload definitions such as layout properties, color structs and utils.

`Signal`s are also defined here.

### `plasticism::atom`

*Atom* means basic component or visual element in Plasticism. All widgets are composed with atoms.

For example, a button may be composed with a `Rect` and a `Label`.

Invocations of atoms are *functional*.

Todo atoms:

- [ ] `Rect`
- [ ] `Label`
- [ ] `Shape`
- [ ] `Icon`
- [ ] `Image`

### `plasticism::processor`

`Processor`s are minium units that handle `Signal`s and generate them.

For example, a `DragProcessor` receives `MouseDown`, `MouseMoved` and `MouseUp`, and generates `DragStarted`, `Dragged` and `DragStopped`. We can also read state properties from `Processor`s with a `StateMachine`, such as `dragging: bool`.

Todo processors:

- [ ] `ButtonProcessor`
- [ ] `DragProcessor`
- [ ] `ScrollProcessor`

### `plasticism::widget`

Widgets can be composed with `atom`s, with optional children. They control event handling, layout, updating and drawing.

See docs in code files for more.
