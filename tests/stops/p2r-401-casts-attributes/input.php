<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// casts() is left out of the output only with no attribute (P2R-401), because PHP checks an
// attribute's arguments and an #[\Override] when it loads the class. Main converted each
// of these models.

// The stand-in Model has no casts(), as Laravel 10's has not, so #[\Override] overrides
// nothing and PHP refuses it.
final class Overriding extends Model
{
    protected $table = 'codes';

    public $timestamps = false;

    #[\Override]
    protected function casts(): array
    {
        return ['code' => 'string'];
    }
}

// static::class is no constant expression, so PHP refuses it in an attribute argument.
final class StaticName extends Model
{
    protected $table = 'codes';

    public $timestamps = false;

    #[\Deprecated(static::class)]
    protected function casts(): array
    {
        return ['code' => 'string'];
    }
}

// Neither is a function call.
final class Called extends Model
{
    protected $table = 'codes';

    public $timestamps = false;

    #[\Deprecated(strtoupper('old'))]
    protected function casts(): array
    {
        return ['code' => 'string'];
    }
}

// PHP refuses a positional argument after a named one.
final class Positional extends Model
{
    protected $table = 'codes';

    public $timestamps = false;

    #[\Deprecated(since: '1.0', 'old')]
    protected function casts(): array
    {
        return ['code' => 'string'];
    }
}

// The form has no &; before, only a Mago finding stopped this one.
final class ByReference extends Model
{
    protected $table = 'codes';

    public $timestamps = false;

    protected function &casts(): array
    {
        return ['code' => 'string'];
    }
}

// PHP accepts this one, but the form has no attribute, so it stops too.
final class Marked extends Model
{
    protected $table = 'codes';

    public $timestamps = false;

    #[\Deprecated]
    protected function casts(): array
    {
        return ['code' => 'string'];
    }
}
