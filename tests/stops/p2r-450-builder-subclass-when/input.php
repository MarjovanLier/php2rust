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

// query() gives back a subclass with its own when(), which Mago does not see (P2R-450).
/**
 * @return Collection<int, Code>
 */
function filtered(?string $name): Collection
{
    return Code::query()->when($name, fn (Builder $query, string $name) => $query->where('name', $name))->get();
}
