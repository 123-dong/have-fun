export default function StreamStatus({
  streamOn,
  streamStatus,
  onToggle,
}: any) {
  return (
    <div className="flex items-center space-x-4">
      <button
        onClick={onToggle}
        className={`px-4 py-2 rounded text-white ${
          streamOn ? "bg-red-500" : "bg-green-500"
        }`}
      >
        {streamOn ? "Stop Stream" : "Start Stream"}
      </button>
      <span>
        Status:{" "}
        {streamStatus === "connecting"
          ? "Connecting..."
          : streamStatus === "connected"
          ? "Connected"
          : streamStatus === "failed"
          ? "Failed"
          : "Idle"}
      </span>
    </div>
  );
}
