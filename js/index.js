
import './../css/2048.css';
import 'classlist';
import { start } from './application';

import("../pkg").then(module => {

  start();
  module.run();

});
