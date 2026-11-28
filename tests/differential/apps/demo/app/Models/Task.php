<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

final class Task extends Model
{
    protected $fillable = ['project_id', 'title', 'done', 'hours'];

    protected $casts = ['done' => 'boolean'];

    public $timestamps = false;

    /**
     * @return BelongsTo<Project, $this>
     */
    public function project(): BelongsTo
    {
        return $this->belongsTo(Project::class);
    }
}
