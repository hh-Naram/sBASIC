pub const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core

    layout (location = 0) in vec4 a_Position;

    void main()
    {
        gl_Position = a_Position;
    }
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core

    out vec4 a_OutColor;

    void main()
    {
        a_OutColor = vec4(1.0);
    }
"#;
