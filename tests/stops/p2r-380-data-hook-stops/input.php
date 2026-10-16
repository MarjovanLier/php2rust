<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

// laravel-data calls a hook as a public static method.
final class Hidden extends Data
{
    protected static function rules(): array
    {
        return [];
    }
}

final class Instance extends Data
{
    public function authorize(): bool
    {
        return true;
    }
}

// PHP checks #[\Override] against Data, which has no real rules().
final class Marked extends Data
{
    #[\Override]
    public static function rules(): array
    {
        return [];
    }
}

// PHP rejects two methods of one name in any letter case; Mago does not report it.
final class Twice extends Data
{
    public static function rules(): array
    {
        return [];
    }

    public static function RULES(): array
    {
        return [];
    }
}

interface Guarded
{
    public function authorize(): bool;
}

// The interface declares the name, so P2R-371 applies.
final class Implemented extends Data implements Guarded
{
    public static function authorize(): bool
    {
        return true;
    }
}

// A left-out hook is not a method of the class.
final class Calls extends Data
{
    public static function errorBag(): string
    {
        return 'calls';
    }

    public function check(): string
    {
        return self::errorBag() . static::errorBag() . Calls::errorBag() . $this->errorBag();
    }
}

// PHP rejects a bare return in a method with a return type when it compiles the file.
final class Bare extends Data
{
    public static function rules(): array
    {
        return;
    }
}

// The docblock is not part of the hook.
final class Documented extends Data
{
    /** @return NoSuchClass */
    public static function redirect(): string
    {
        return 'home';
    }
}
