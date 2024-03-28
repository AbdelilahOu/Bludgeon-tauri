type groupBy = <T>(list: Record<any, any>[], by: string) => Record<string, T[]>;

export const groupBy: groupBy = (list, by) => {
  if (typeof list !== "object" || !("length" in list)) return {};

  const result = new Map<string, any[]>();

  for (const el of list) {
    if (by in el) {
      const current = result.get(el[by]);
      if (!current) {
        result.set(el[by], [el]);
        continue;
      }
      result.set(el[by], [...current, el]);
    }
  }

  //@ts-ignore
  return Object.fromEntries(result);
};

type keys = (obj: Record<any, any>) => string[];

export const keys: keys = (obj) => {
  if (typeof obj !== "object") return [];

  const result = new Set<string>();
  //@ts-ignore
  for (const [key] of Object.entries(obj)) {
    result.add(key);
  }

  return Array.from(result);
};

type values = (obj: Record<any, any>) => any[];

export const values: values = (obj) => {
  if (typeof obj !== "object") return [];

  const result = [];

  //@ts-ignore
  for (const [_, value] of Object.entries(obj)) {
    result.push(value);
  }

  return result;
};

type mapValues = (obj: Record<any, any>, cb: Function) => Record<any, any>;

export const mapValues: mapValues = (obj, callback) => {
  if (typeof obj !== "object") return {};

  const result = new Map<string, any>();

  //@ts-ignore
  for (const [key, value] of Object.entries(obj)) {
    result.set(key, callback(value));
  }

  //@ts-ignore
  return Object.fromEntries(result);
};

type cFLetter = (text: string) => string;

export const capitalizeFirstLetter: cFLetter = (text) => {
  return text.charAt(0).toUpperCase() + text.slice(1);
};
