import Option from "./option";

export default function OptionsView({ options, chosenOption }) {
  return (
    <div className="options-view interaction-screen__option-view">
      {chosenOption ?
        <Option
          key={chosenOption}
          className="options-view__option"
          label={chosenOption}
          amount={0}
          big
        />
      : options.map((key) => (
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
