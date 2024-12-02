import Option from "./option";
import clsx from "clsx";

export default function OptionsView({ options, tagCount, tagsMap }) {
  const tagsByAntenna = Object.values(tagsMap).reduce(
    (acc, { id, antenna }) => {
      if (!acc[antenna]) {
        acc[antenna] = [];
      }
      acc[antenna].push(id);
      return acc;
    },
    {},
  );
  return (
    <div className="options-view interaction-screen__option-view">
      {options.map((option, index) => (
        <Option
          key={option.value}
          className={clsx(
            "options-view__option",
            option.color && `options-view__option--${option.color}`,
          )}
          label={option.value}
          big={options.length === 1}
          amount={tagCount[index + 1]}
          tagIds={tagsByAntenna[index + 1]}
        />
      ))}
    </div>
  );
}
