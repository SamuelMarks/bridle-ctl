import { ComponentFixture, TestBed } from '@angular/core/testing';
import { LocalFixComponent } from './local-fix.component';
import { ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';

describe('LocalFixComponent', () => {
  let component: LocalFixComponent;
  let fixture: ComponentFixture<LocalFixComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LocalFixComponent, ReactiveFormsModule]
    }).compileComponents();

    fixture = TestBed.createComponent(LocalFixComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should have invalid form initially', () => {
    expect(component.form.valid).toBeFalse();
  });

  it('should emit fix event with parsed args and dryRun', () => {
    spyOn(component.fix, 'emit');

    component.form.setValue({
      pattern: 'TODO',
      tools: 'sed, replace',
      args: '{"replace": {"new_string": "FIXED"}}',
      dryRun: false
    });

    component.onSubmit();

    expect(component.fix.emit).toHaveBeenCalledWith({
      pattern: 'TODO',
      tools: ['sed', 'replace'],
      args: { replace: { new_string: 'FIXED' } },
      dryRun: false
    });
  });

  it('should handle invalid JSON in args', () => {
    spyOn(component.fix, 'emit');

    component.form.setValue({
      pattern: 'TODO',
      tools: '',
      args: '{invalid json}',
      dryRun: true
    });

    component.onSubmit();

    expect(component.fix.emit).toHaveBeenCalledWith({
      pattern: 'TODO',
      tools: [],
      args: {},
      dryRun: true
    });
  });

  it('should fallback to dryRun true if null', () => {
    spyOn(component.fix, 'emit');

    component.form.setValue({
      pattern: 'TODO',
      tools: '',
      args: '{}',
      dryRun: null
    });

    component.onSubmit();

    expect(component.fix.emit).toHaveBeenCalledWith({
      pattern: 'TODO',
      tools: [],
      args: {},
      dryRun: true
    });
  });
  it('should set operating state', () => {
    expect(component.isOperating()).toBeFalse();
    component.setOperating(true);
    expect(component.isOperating()).toBeTrue();
  });

  it('should display result panel with modified files and diff when result is set', () => {
    component.setResult({ 
      output: 'Fix completed',
      modifiedFiles: ['src/main.ts'],
      diff: '--- src/main.ts\n+++ src/main.ts'
    });
    fixture.detectChanges();
    
    const preElements = fixture.debugElement.queryAll(By.css('pre'));
    expect(preElements.length).toBe(2);
    expect(preElements[0].nativeElement.textContent).toContain('Fix completed');
    expect(preElements[1].nativeElement.textContent).toContain('--- src/main.ts');
    
    const listItems = fixture.debugElement.queryAll(By.css('.file-list li'));
    expect(listItems.length).toBe(1);
    expect(listItems[0].nativeElement.textContent).toContain('src/main.ts');
  });
});
