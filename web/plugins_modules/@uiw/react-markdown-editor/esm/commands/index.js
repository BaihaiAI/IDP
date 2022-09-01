import { bold } from './bold';
import { italic } from './italic';
import { header } from './header';
import { strike } from './strike';
import { underline } from './underline';
import { olist } from './olist';
import { ulist } from './ulist';
import { link } from './link';
import { todo } from './todo';
import { image } from './image';
import { fullscreen } from './fullscreen';
import { preview } from './preview';
export var defaultCommands = {
  bold,
  italic,
  header,
  strike,
  underline,
  olist,
  ulist,
  link,
  todo,
  image,
  fullscreen,
  preview
};
export var getCommands = () => Object.keys(defaultCommands).filter(key => !/^(fullscreen|preview)/.test(key)).map(key => defaultCommands[key]);
export var getModeCommands = () => [preview, fullscreen];
//# sourceMappingURL=index.js.map