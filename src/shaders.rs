pub const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core

    layout (location = 0) in vec4 a_Position;
    uniform mat4 u_MVP;

    void main()
    {
        gl_Position = a_Position * u_MVP;
    }
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core

    out vec4 a_OutColor;
    uniform vec4 u_Color;

    void main()
    {
        a_OutColor = u_Color;
    }
"#;
