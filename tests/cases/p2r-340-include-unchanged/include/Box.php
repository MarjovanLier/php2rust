<?php

declare(strict_types=1);

// The input declares Box too; the input's Box wins.
final class Box
{
    public function label(): string
    {
        return 'vendored';
    }
}
