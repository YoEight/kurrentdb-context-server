use serde::Serialize;

#[derive(Serialize)]
pub struct Argument {
    pub name: &'static str,
    #[serde(rename = "description")]
    pub desc: &'static str,
    pub required: bool,
}

#[derive(Serialize)]
pub struct Prompt {
    pub name: &'static str,
    #[serde(rename = "description")]
    pub desc: &'static str,

    #[serde(rename = "arguments")]
    pub args: &'static [Argument],
}

pub static PROMPTS: &'static [Prompt] = &[
    Prompt {
        name: "read_stream",
        desc: "Asks the LLM to read a stream",
        args: &[Argument {
            name: "stream_name",
            desc: "Name of the stream to read",
            required: true,
        }],
    },
    Prompt {
        name: "append_stream",
        desc: "Asks the LLM to append a stream",
        args: &[
            Argument {
                name: "stream_name",
                desc: "Name of the stream to append",
                required: true,
            },
            Argument {
                name: "payload",
                desc: "Event to append",
                required: true,
            },
        ],
    },
];
