import { defineComponent, ref, type PropType } from "vue";

export const UiInput = defineComponent({
  name: "UiInput",
  props: {
    Type: {
      type: String,
      default: "text",
    },
    Disable: {
      type: Boolean,
      default: false,
    },
    OnInputChange: {
      type: Function as PropType<(value: string | number) => void>,
      required: true,
    },
    PlaceHolder: {
      type: String,
    },
    IsEmpty: {
      type: Boolean,
      required: true,
    },
  },
  setup(props, { slots }) {
    const emitChange = (e: Event) => {
      props.OnInputChange(e.target?.value as string | number);
    };
    return () => (
      <div
        class={`flex flex-nowrap h-full items-center border-2 rounded-md transition-all duration-200 ${
          props.IsEmpty && "border-red-300 placeholder:text-red-300"
        }`}
      >
        {slots.default?.()}
        <input
          class={`defaultInput border-none ${
            props.IsEmpty && "placeholder:text-red-300"
          }`}
          disabled={props.Disable}
          onInput={emitChange}
          type={props.Type}
          placeholder={props.PlaceHolder}
        />
      </div>
    );
  },
});
