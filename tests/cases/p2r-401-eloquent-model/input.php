<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;

enum Status: string
{
    case OPEN = 'open';
}

// Its table is location_codes. The column exists is Model's own property, active is an
// uncast boolean, price a decimal and status cast to an enum, so all four are left out.
final class LocationCode extends Model
{
    use HasFactory, SoftDeletes;

    protected $fillable = ['code', 'status'];

    protected $guarded = [];

    protected $hidden = ['price'];

    protected $casts = [
        'status' => Status::class,
    ];

    public $timestamps = false;
}

// categories, from a y after a consonant.
class Category extends Model
{
}

// boxes, from a word ending in x.
final class Box extends Model
{
}

final class LegacyItem extends Model
{
    protected $table = 'legacy_items';
}

function describe(LocationCode $location, Box $box, LegacyItem $item): string
{
    return $location->code . ' ' . $box->size . ' ' . $item->body;
}

function categoryLabel(Category $category): ?string
{
    return $category->label;
}
