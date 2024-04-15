import Option from "./option";

export default function OptionsView({ options }) {
  return (
    <div className="options-view interaction-screen__option-view">
      {options.map((key) => (
        <Option
          key={key}
          className="options-view__option"
          label={key}
          amount={0}
        />
      ))}
    </div>
  );
}
