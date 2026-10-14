<?php

declare(strict_types=1);

namespace App\Dates;

final readonly class Parts
{
    public function __construct(
        public string $day,
    ) {}

    /**
     * @param array{day: string} $data
     */
    public static function fromArray(array $data): self
    {
        return new self($data['day']);
    }
}
