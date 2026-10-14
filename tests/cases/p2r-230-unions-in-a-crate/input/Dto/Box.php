<?php

declare(strict_types=1);

namespace App\Dto;

final readonly class Box
{
    public function __construct(public int|string $id) {}

    public function id(): int|string
    {
        return $this->id;
    }

    public function kind(): string
    {
        $id = $this->id;
        return gettype($id);
    }
}
