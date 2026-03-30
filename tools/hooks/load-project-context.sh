#!/bin/bash
# Gang of Bastards - Smart Project Context Hook
# Throttled execution to prevent spam

CACHE_FILE="/tmp/.claude_context_cache"
CACHE_TTL=300  # 5 minutes

# Check if cache is fresh
if [[ -f "$CACHE_FILE" ]] && [[ $(($(date +%s) - $(stat -f %m "$CACHE_FILE"))) -lt $CACHE_TTL ]]; then
    # Use cached output
    cat "$CACHE_FILE"
    exit 0
fi

# Generate fresh context
{
    echo "🚀 Remote service health checks:"
    echo ""
    
    # Quick parallel health checks (max 2 seconds each)
    declare -A endpoints=(
        ["Vista"]="https://api.libraxis.cloud/health"
        ["LLM"]="https://api.libraxis.cloud/llm/v1/health"  
    )
    
    for service in "${!endpoints[@]}"; do
        url="${endpoints[$service]}"
        if timeout 2s curl -s -f "$url" > /dev/null; then
            echo "✅ $service OK"
        else
            echo "❌ $service DOWN"  
        fi
    done
    
    echo ""
    echo "💾 System resources:"
    echo "   Memory: $(top -l 1 | grep PhysMem | awk '{print $2, $4, $6}')"
    echo ""
    
} > "$CACHE_FILE"

cat "$CACHE_FILE"