export default function ErrorScreen({ errorList }) {
  return (
    <div className="error-screen">
      <svg
        width="163"
        height="162"
        viewBox="0 0 163 162"
        xmlns="http://www.w3.org/2000/svg"
      >
        <circle cx="81.8408" cy="81" r="80.9366" fill="currentColor" />
        <path
          d="M54.011 115.785L47.0536 108.828L74.8833 80.9981L47.0536 53.1684L54.011 46.2109L81.8408 74.0407L109.671 46.2109L116.628 53.1684L88.7982 80.9981L116.628 108.828L109.671 115.785L81.8408 87.9556L54.011 115.785Z"
          fill="#CE2626"
        />
      </svg>
      <div className="error-screen__content">
        <h1>ERROR</h1>
        <h2>The following errors occured:</h2>
        <ol className="error-screen__list">
          {errorList.map((error) => (
            <li key={error.message}>
              Kind: {error.kind} <br /> Message: {error.message}
            </li>
          ))}
        </ol>
        <button type="button" onClick={() => window.location.reload()}>
          Reload
        </button>
      </div>
    </div>
  );
}
