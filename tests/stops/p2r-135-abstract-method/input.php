<?php

declare(strict_types=1);

// PHP allows an abstract method only in an abstract class or a trait, so the refused
// abstract class always comes with it.
abstract class Shape
{
    abstract public function area(): int;
}

echo "shape\n";
