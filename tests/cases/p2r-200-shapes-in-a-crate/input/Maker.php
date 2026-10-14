<?php

declare(strict_types=1);

namespace App;

use App\Dates\Parts;

final readonly class Maker
{
    /**
     * @return array{day: string}
     */
    public static function make(string $day): array
    {
        return ['day' => $day];
    }

    public static function parts(string $day): Parts
    {
        return Parts::fromArray(self::make($day));
    }

    /**
     * @param array{day: string} $d
     */
    public static function show(array $d): string
    {
        return $d['day'];
    }
}
