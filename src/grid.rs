use crate::tic_tac_toe::{CellValue, TicTacToe};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    MakePlay(u8, u8),
}

pub struct Grid {
    link: ComponentLink<Self>,
    tic_tac_toe: TicTacToe,
    current_player: CellValue,
}

impl Component for Grid {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            tic_tac_toe: TicTacToe::new(),
            current_player: CellValue::Cross,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakePlay(x, y) => {
                self.tic_tac_toe
                    .set_value(self.current_player, x.into(), y.into());

                // Flip current player!
                if self.current_player == CellValue::Circle {
                    self.current_player = CellValue::Cross;
                } else {
                    self.current_player = CellValue::Circle;
                }

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <table>
            { for self.tic_tac_toe.get_state().iter().enumerate().map(|(i, row)| self.render_row(i as u8, row)) }
          </table>
        }
    }
}

impl Grid {
    fn render_cell(&self, i: u8, j: u8, cell_value: Option<CellValue>) -> Html {
        html! {
          <td>{
            match cell_value {
              Some(CellValue::Cross) => html!{<span>{"X"}</span>},
              Some(CellValue::Circle) => html!{<span>{"O"}</span>},
              None => html!{
                <button onclick=self.link.callback(move |_| Msg::MakePlay(i, j))>
                  {"?"}
                </button>}
            }
          }</td>
        }
    }

    fn render_row(&self, i: u8, row_values: &[Option<CellValue>]) -> Html {
        html! {
          <tr>
            { for row_values.iter().enumerate().map(move |(j, val)| self.render_cell(i, j as u8, *val)) }
          </tr>
        }
    }
}
