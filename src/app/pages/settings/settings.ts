import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { TranslatePipe } from '@ngx-translate/core';

@Component({
    selector: 'app-settings',
    imports: [CommonModule, TranslatePipe],
    templateUrl: './settings.html',
    styleUrl: './settings.css',
})
export class Settings {
    importFile!: File | null;

    message = '';
    messageType: 'success' | 'error' | '' = '';

    constructor() {}

    onImportFile(event: any) {
        this.importFile = event.target.files[0] ?? null;
    }

    importData() {}

    exportData() {}

    resetAll() {}

    private showMessage(msg: string, type: 'success' | 'error') {}
}
