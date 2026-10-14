<?php

declare(strict_types=1);

final readonly class Tag
{
    const class = 'tag';

    public function __construct(public int $n) {}
}

echo (new Tag(1))->n, "\n";
