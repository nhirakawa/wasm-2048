
import './../css/2048.css';
import 'classlist';
import { KeyboardInputManager } from './keyboard_input_manager';
import { GameManager } from './game_manager';
import { HTMLActuator } from './html_actuator';
import { LocalStorageManager } from './local_storage_manager';
import { Tile } from './tile';

const width = 4;
const height = 4;

import("../pkg").then(module => {

  module.run();

  let manager = module.Manager.new(width, height);
  let tiles = [];
  for (var i = 0; i < width * height; i++) {
    let tile = manager.get_tile(i);

    let x = tile.get_x();
    let y = tile.get_y();
    let value = tile.get_value();

    if (value) {
      tiles.push(new Tile({ x, y }, value));
    }
  }

  console.log(tiles);

  let game_manager = new GameManager(4, KeyboardInputManager, HTMLActuator, LocalStorageManager, manager);
  game_manager.setStartTiles(tiles);
});
