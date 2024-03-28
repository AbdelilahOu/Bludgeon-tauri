export type Res<T> = {
  error?: string;
  message?: string;
  data: T;
};
