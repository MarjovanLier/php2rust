<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;

// A cast gives a column its type, whatever the migration declared; casts() wins over
// $casts on a shared key.
final class Flag extends Model
{
    protected $casts = [
        'enabled' => 'boolean',
        'visible' => 'bool',
        'counted' => 'string',
        'weight' => 'string',
        'ratio' => 'decimal:2',
        'updated_at' => 'string',
    ];

    protected function casts(): array
    {
        return [
            'counted' => 'integer',
            'opened_at' => 'string',
            'code' => 'int',
        ];
    }
}

// Without a cast of its own, Laravel reads created_at and updated_at as dates while
// timestamps are on, and deleted_at as a date in a model that uses SoftDeletes.
final class Stamp extends Model
{
    use SoftDeletes;

    protected $table = 'flags';
}

final class Untimed extends Model
{
    protected $table = 'flags';

    public $timestamps = false;
}

function summary(Flag $flag): string
{
    return $flag->weight . '/' . $flag->opened_at . '/' . ($flag->counted + $flag->code);
}

function shown(Flag $flag): bool
{
    return $flag->enabled;
}

function maybeShown(Flag $flag): ?bool
{
    return $flag->visible;
}
