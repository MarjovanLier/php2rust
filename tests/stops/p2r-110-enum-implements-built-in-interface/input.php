<?php

declare(strict_types=1);

enum Size: int implements Countable
{
    case Small = 1;
    case Large = 3;

    public function count(): int
    {
        return $this->value;
    }
}

echo Size::Large->value, "\n";
