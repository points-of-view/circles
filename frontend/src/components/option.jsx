import Shape from "./shape";

export default function Option({ label, amount = 0 }) {
  return (
    <div className="option">
      <div className="option__amount">{amount}</div>
      <div className="option__content">
        <div className="option__label">{label}</div>
          <div className="option__figure-container">
            {[...Array(amount)].map((value, index) => (
              <Shape key={index} shape={"figure"} />
            ))}
          </div>
      </div>
    </div>
  );
}
