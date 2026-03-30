# Gemini CLI stream-json filter (verified against real output 2026-03-27)
# Event types: init, message (user/assistant+delta), tool_use, tool_result, result

def stamp: (now | strftime("%H:%M:%S"));
def tool_tag($name): "\u001b[36m[" + stamp + " " + $name + "]\u001b[0m ";

if .type == "init" then
  "\u001b[33m[" + stamp + "] session: " + (.session_id // "?") + "\u001b[0m"
  + (if .model then " (" + .model + ")" else "" end) + "\n"

elif .type == "message" then
  if .role == "assistant" then
    (.content // "")
  else empty end

elif .type == "tool_use" then
  "\n" + tool_tag(.tool_name // .name // "?")

elif .type == "tool_result" then
  (.output // "") as $out |
  if ($out | length) > 0 then
    ($out | split("\n")) as $lines |
    if ($lines | length) > 12 then
      "\u001b[2m" + ($lines[0:5] | join("\n")) + "\n  ... (" + ($lines | length | tostring) + " lines)\u001b[0m\n"
    elif ($out | length) > 500 then
      "\u001b[2m" + $out[0:400] + " ...\u001b[0m\n"
    else
      "\u001b[2m" + $out + "\u001b[0m\n"
    end
  else empty end

elif .type == "error" then
  "\u001b[31m[" + stamp + " error] " + (.message // .error // "unknown") + "\u001b[0m\n"

elif .type == "result" then
  "\n\u001b[32m[" + stamp + "] " + (.status // "done") + "\u001b[0m"
  + (if .stats then
      " \u001b[2m" + (.stats.input_tokens | tostring) + " in / "
      + (.stats.output_tokens | tostring) + " out"
      + " / " + (.stats.duration_ms | tostring) + "ms"
      + (if .stats.tool_calls then " / " + (.stats.tool_calls | tostring) + " tools" else "" end)
      + "\u001b[0m"
    else "" end)
  + "\n"

else empty end
