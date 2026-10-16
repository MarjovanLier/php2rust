<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

/**
 * @return Collection<int, Code>
 */
function filtered(?string $name): Collection
{
    return Code::query()
        ->when($name, static function (Builder $query, string $name): void {
            $query->where('name', $name);
        })
        ->get();
}
