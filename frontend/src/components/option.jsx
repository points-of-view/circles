import Shape from "./shape";

export default function Option({ label, amount = 0, option }) {
  return (
    <div
      className="option"
      style={{
        "--option-color": `var(--option-color-${option.key})`,
      }}
    >
      <div className="option__amount">{amount}</div>
      <div className="option__content" >
        <div className="option__label">{label}</div>
        {amount != 0 && (
          <div className="option__figure-container">
            {[...Array(amount)].map((value, index) => (
              <Shape key={index} shape={"figure"}/>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
