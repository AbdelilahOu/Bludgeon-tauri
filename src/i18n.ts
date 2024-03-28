import messages from "@intlify/unplugin-vue-i18n/messages";
import { createI18n } from "vue-i18n";

const locale = localStorage.getItem("locale");

const DEFAULT_DATE_TIME_FORMAT = {
  short: {
    year: "numeric",
    month: "short",
    day: "numeric",
  },
  monthOnly: {
    month: "long",
  },
  long: {
    year: "numeric",
    month: "short",
    day: "numeric",
    weekday: "short",
    hour: "numeric",
    minute: "numeric",
  },
};

// const DEFAULT_NUMBER_FORMATS = {
//   US: {
//     currency: {
//       style: "currency",
//       currency: "USD",
//       notation: "standard",
//     },
//     decimal: {
//       style: "decimal",
//       minimumFractionDigits: 2,
//       maximumFractionDigits: 2,
//     },
//     percent: {
//       style: "percent",
//       useGrouping: false,
//     },
//   },
//   EU: {
//     currency: {
//       style: "currency",
//       currency: "EUR",
//       notation: "standard",
//     },
//     decimal: {
//       style: "decimal",
//       minimumFractionDigits: 2,
//       maximumFractionDigits: 2,
//     },
//     percent: {
//       style: "percent",
//       useGrouping: false,
//     },
//   },
//   AR: {
//     currency: {
//       style: "currency",
//       currency: "MAD",
//       notation: "standard",
//     },
//     decimal: {
//       style: "decimal",
//       minimumFractionDigits: 2,
//       maximumFractionDigits: 2,
//     },
//     percent: {
//       style: "percent",
//       useGrouping: false,
//     },
//   },
// };

const datetimeFormats = {
  "en-US": DEFAULT_DATE_TIME_FORMAT,
  "fr-FR": DEFAULT_DATE_TIME_FORMAT,
  "ar-AE": DEFAULT_DATE_TIME_FORMAT,
  "de-DE": DEFAULT_DATE_TIME_FORMAT,
};

// const numberFormats = {
//   "en-US": DEFAULT_NUMBER_FORMATS.US,
//   "fr-FR": DEFAULT_NUMBER_FORMATS.EU,
//   "ar-AE": DEFAULT_NUMBER_FORMATS.EU,
//   "de-DE": DEFAULT_NUMBER_FORMATS.AR,
// };

export const i18n = createI18n({
  legacy: false,
  globalInjection: false,
  locale: locale ? JSON.parse(locale).key : "en-US",
  fallbackLocale: "en-US",
  availableLocales: ["en-US", "fr-FR", "ar-AE", "de-DE"],
  messages: messages,
  // @ts-ignore
  datetimeFormats,
});
