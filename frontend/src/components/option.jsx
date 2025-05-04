import clsx from "clsx";
import Token from "./token";

export default function Option({
  className = "",
  label,
  amount = 0,
  big = false,
  tagIds = [],
  longOptionText = false,
}) {
  return (
    <div className={clsx("option", className, { "option--big": big })}>
      {!big && <div className="option__amount">{amount}</div>}
      <div className="option__content">
        <div
          className={clsx("option__label", {
            "option__label--small-text": longOptionText,
          })}
        >
          {label}
        </div>
        {!big && (
          <div className="option__figure-container">
            {tagIds.map((value) => (
              <Token tagID={value} key={value} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
