use yew::prelude::*;
use web_sys::{HtmlInputElement};

enum Msg {
    SetAnswer(String),
    Judge,
}

#[derive(Default)]
// ルートコンポーネントが持つ変数
struct Model {
    collect: &'static str,
    answer: String,
    answers: Vec<String>,
    message: &'static str,
    message_color: &'static str,
}

// Componentを実装するとhtmlマクロの中で呼べるようになる
// Propertiesにchildrenフィールド(yew::Children)があると子コンポーネントを持てる
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    // コンポーネントの初期化
    // Defaultトレイトを実装していると::default()が呼べる
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            collect: "hello",
            message: "Let's try!",
            ..Self::default()
        }
    }

    // 戻り値のbooleanは画面更新の有無
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetAnswer(a) => {
                self.answer = a;
                true
            }
            Msg::Judge => {
                if self.answer == self.collect {
                    self.message = "Congratulations!!";
                    self.message_color = "message_correct";
                } else {
                    self.message = "Try Again!!";
                    self.message_color = "message_incorrect"
                }
                self.answers.push(self.answer.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // メッセージ等を送るコンポーネントのスコープを提供
        let link = ctx.link();
        html! {
            <>
                <h1>{ "WORDLE by Yew" }</h1>
                <hr />
                <br/>

                <h2>{"hello"}</h2>

                <div class="input-form">
                    <input oninput={
                        link.callback(|e: InputEvent| Msg::SetAnswer(e.target_unchecked_into::<HtmlInputElement>().value()))
                    }/>
                    <button onclick={link.callback(|_| Msg::Judge)}>
                        { "Judge!!" }
                    </button>
                </div>
                <p class={self.message_color}>
                    {self.message}
                </p>

                <hr />

                {
                    self.answers.iter().map(|answer| {
                        html!{
                            <div>
                                <p class="answer">
                                    {answer}
                                </p>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </>
        }
    }
}

fn main() {
    // <body>タグにマウント
    yew::start_app::<Model>();
}
