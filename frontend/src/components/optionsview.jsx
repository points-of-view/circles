import Option from "./option";

export default function OptionsView({ options, tagsMap, showDescriptionLayout = false }) {
  return (
    <div className="options-view interaction-screen__option-view">
      {Array.isArray(options) ? (
        options.map((key, index) => (
          <Option
            key={key}
            className="options-view__option"
            label={key}
            amount={tagsMap[index + 1]}
          />
        ))
      ) : (
        <Option
          key={options}
          className="options-view__option"
          label={options}
          showDescriptionLayout={showDescriptionLayout}
          big
        />
      )}
    </div>
  );
}
