import Option from "./option";

export default function OptionsView({ options, chosenOption }) {
  console.log(chosenOption)
  console.log(options.find((key) => (key === chosenOption)))
  return (
    <div className="options-view interaction-screen__option-view">
      {chosenOption ? options.find((key) =>
        <Option
          key={key === chosenOption}
          className="options-view__option"
          label={key === chosenOption}
          amount={0}
        />)
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
