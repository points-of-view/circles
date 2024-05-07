import NL from "./nl.json";
import EN from "./en.json";
import PO from "./po.json";
import SL from "./sl.json";

const LOCALES = {
  NL,
  EN,
  SL,
  PO,
};

export default function translate(key, language = "EN") {
  let translation;

  if (LOCALES[language.toUpperCase()][key]) {
    translation = LOCALES[language.toUpperCase()][key];
  } else if (LOCALES["EN"][key]) {
    translation = LOCALES["EN"][key];
  } else {
    translation = key;
  }
  return translation;
}

/**
 * Resolve an error from our backend to a translations
 *
 * @param {CirclesError} error
 * @returns string
 */
export function translateError(error, language) {
  return translate(`error_${error.error_type}_${error.kind}`, language);
}
