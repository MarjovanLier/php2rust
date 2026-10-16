<?php

declare(strict_types=1);

namespace Illuminate\Support\Traits;

// The input's own Conditionable, which wins over the vendored one (P2R-340): the tool tells
// Mago nothing about when() (P2R-450), and the links stop even though this when() returns
// $this (P2R-451).
trait Conditionable
{
    /**
     * @template TWhenParameter
     *
     * @param  TWhenParameter|null  $value
     * @param  (callable($this, TWhenParameter): mixed)|null  $callback
     * @return $this
     */
    public function when($value = null, ?callable $callback = null)
    {
        return $this;
    }
}
