// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2022
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** APACHE 2.0 LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Licensed under the Apache License, Version 2.0 (the "License");
// * you may not use this file except in compliance with the License.
// * You may obtain a copy of the License at
// *
// *     http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a copy
// * of this software and associated documentation files (the "Software"), to deal
// * in the Software without restriction, including without limitation the rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

use super::errors::GameError;
use super::player::Player;
use super::player::Symbol;
use crate::components::Board;

#[allow(dead_code)]
pub struct Game {
    player_x: Player,
    player_o: Player,
    board: Board,
    winner: Option<Player>,
}

impl Game {
    pub fn new() -> Self {
        let current_player = Player { symbol: Symbol::X };
        let other_player = Player { symbol: Symbol::O };
        Self {
            player_x: current_player,
            player_o: other_player,
            board: Board::new(),
            winner: None,
        }
    }

    #[allow(dead_code)]
    pub fn play(&mut self) -> Result<(), GameError> {
        self.board.get_square(2, 2).set_value(Symbol::X);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}
