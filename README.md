# Разработка графики на языке rust

Я решил разобраться в теме рендеринга 3D сцен, поэтому решил записать цикл уроков по этой теме.
Во многом это будет перевод официального [туториала](https://sotrh.github.io/learn-wgpu/#what-is-wgpu) по _wgpu_ + мои комментарии.

### Мотивация
Почему именно _wgpu_, а не _OpenGL_, _Vulkan_ или _DirectX_? Я за кроссплатформенную разработку, а wgpu поддерживает несколько графический backend-ов, операционных систем и даже компилируется в _webgl_ (то-есть, мы можем сделать игру в браузере).
Кроме того, стандарт _WebGPU_ (на чем основан _wgpu_) мне видится многообещающим, за ним будущее.

### Что такое WGPU?

[Wgpu](https://github.com/gfx-rs/wgpu) это реализация спецификации _WebGPU_ на языке _rust_, целью которой является предоставить более безопасный и удобный доступ к функционалу видео карты из браузера (замена _webgl_).
Во многом, API перекликается с таковым у Vulkan API, предоставляя также возможность _трансляции_ в другие backend-ы (_DirectX_, _Metal_, _Vulkan_).

## Приступим!

Как правило, любая игра начинается с окна, именно в нем в дальнейшем можно отрисовывать результаты работы видеокарты.

Сделайте новый проект с помощью _cargo_:
```
cargo new rust_wgpu_tutorial --bin
```

Я буду использовать следующие зависимости:
```toml
[dependencies]
winit = "0.26"
env_logger = "0.9"
log = "0.4"
wgpu = "0.13"
```

Теперь сам код:
```rust
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
```

Помимо самого окна я добавил еще логгер, чтобы в дальнейшем видеть детализацию ошибок _wgpu_, если они произойдут.
Если вы работали с растом, то этот код не вызывает много вопросов, кроме разве что конструкции внутри `match`.
Там говорится следующее: для всех событий в `event_loop`, отбери только те, которые относятся к текущему окну. 
Если событие `WindowEvent::CloseRequested`, либо `WindowEvent::KeyboardInput`, тогда происходит деструктуризация структуры `KeyboardInput`. 
Если поле `virtual_keycode` внутри равно `Some(VirtualKeyCode::Escape)`, тогда установи событие `ControlFlow::Exit` (закрой окно).
Напоминает продвинутый pattern-matching в _haskell_. Вот за что я люблю _rust_.