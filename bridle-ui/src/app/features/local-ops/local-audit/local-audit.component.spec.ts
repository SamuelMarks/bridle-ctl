import { ComponentFixture, TestBed } from '@angular/core/testing';
import { LocalAuditComponent } from './local-audit.component';
import { ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';

describe('LocalAuditComponent', () => {
  let component: LocalAuditComponent;
  let fixture: ComponentFixture<LocalAuditComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LocalAuditComponent, ReactiveFormsModule]
    }).compileComponents();

    fixture = TestBed.createComponent(LocalAuditComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should have invalid form initially', () => {
    expect(component.form.valid).toBeFalse();
  });

  it('should emit audit event with parsed args', () => {
    spyOn(component.audit, 'emit');

    component.form.setValue({
      pattern: 'TODO',
      tools: 'grep, eslint',
      args: '{"grep": {"flags": "-i"}}'
    });

    component.onSubmit();

    expect(component.audit.emit).toHaveBeenCalledWith({
      pattern: 'TODO',
      tools: ['grep', 'eslint'],
      args: { grep: { flags: '-i' } }
    });
  });

  it('should emit audit event with empty args if parsing fails', () => {
    spyOn(component.audit, 'emit');

    component.form.setValue({
      pattern: 'FIXME',
      tools: '',
      args: '{invalid json}'
    });

    component.onSubmit();

    expect(component.audit.emit).toHaveBeenCalledWith({
      pattern: 'FIXME',
      tools: [],
      args: {}
    });
  });

  it('should set operating state', () => {
    expect(component.isOperating()).toBeFalse();
    component.setOperating(true);
    expect(component.isOperating()).toBeTrue();
  });

  it('should display result panel when result is set', () => {
    component.setResult({ output: 'Found 1 match' });
    fixture.detectChanges();
    
    const pre = fixture.debugElement.query(By.css('pre')).nativeElement;
    expect(pre.textContent).toContain('Found 1 match');
  });

  it('should not display result panel when result is null', () => {
    component.setResult(null);
    fixture.detectChanges();
    
    const panel = fixture.debugElement.query(By.css('.result-panel'));
    expect(panel).toBeNull();
  });
});
