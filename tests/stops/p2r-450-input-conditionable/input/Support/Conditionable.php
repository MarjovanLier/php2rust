<?php

declare(strict_types=1);

namespace Illuminate\Support\Traits;

// The input's own Conditionable, which wins over the vendored one (P2R-340), so the tool
// tells Mago nothing about when() and the chain stays mixed (P2R-450).
trait Conditionable
{
    /**
     * @template TWhenParameter
     * @template TWhenReturnType
     *
     * @param  TWhenParameter|null  $value
     * @param  (callable($this, TWhenParameter): TWhenReturnType)|null  $callback
     * @return $this|TWhenReturnType
     */
    public function when($value = null, ?callable $callback = null)
    {
        return $this;
    }
}
