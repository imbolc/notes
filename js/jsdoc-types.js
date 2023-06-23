// Typing using JSDoc
//
//
// Checking is possible with eslint: `npm i -D eslint eslint-plugin-jsdoc`
// and following `.eslintrc.json`:
// {
//   "parserOptions": {
//     "ecmaVersion": 2020,
//    "sourceType": "module"
//   },
//   "plugins": ["jsdoc"],
//   "extends": ["eslint:recommended", "plugin:jsdoc/recommended"]
// }

/**
 * Function with arguments and a return type
 *
 * @param {string} name - User name
 * @param {number[]} friends - An array of friend ids
 * @returns {{name: string, friends: number[]}} an user object
 */
export function create_user(name, friends) {
  return { name, friends };
}

/**
 * Priminive types and built-in types: string, number, boolean, object, Array
 *
 * @type {string} name
 */
export let name;

/**
 * Optional (nullable) types
 *
 * @type {?string} an optional string
 */
export let maybe_name;
