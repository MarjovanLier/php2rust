<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

// A real method of Data makes these overrides, which P2R-350 stops.
final class Account extends Data
{
    public function __construct(public string $number) {}

    /** @return array<string, mixed> */
    public static function rules(): array
    {
        return [];
    }

    public static function authorize(): bool
    {
        return true;
    }

    // Mago takes the @method tag of Data over the real method of its trait.
    public static function errorBag(): string
    {
        return 'account';
    }
}
